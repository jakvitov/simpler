import sys
import os

sys.path.insert(0, os.path.abspath("./libs"))


import requests
import os
from enum import Enum
from datetime import timedelta


class SolverType(Enum):
    BASIC = "basic"
    TWO_PHASE = "two-phase"
    REVISED = "revised"
    MULTIPLICATIVE = "multiplicative"

class OptimisationTarget(Enum):
    MAX = "MAX"
    MIN = "MIN"

class SolverResponse(Enum):
    OK = "✅"
    ERROR = "⚠️"
    IGNORED = "ℹ️"
    

BE_URL = "http://localhost:8080/api/simpler/solve-lp/"
EVALUATION_THRESHOLD_MS = 10000 

solverConfiguration = {
    "basicSimplexMaxIterations": 100000,
    "basicSimplexMaxBaseCycles": 100000,
    "twoPhaseMaxIterations": 100000,
    "twoPhaseMaxBaseCycles": 100000,
    "revisedMaxIterations": 100000,
    "revisedMaxBaseCycles": 100000,
    "multiplicativeMaxIterations": 100000,
    "multiplicativeMaxBaseCycles": 100000,
}


def call_solve_api_with_mps(mps: str, filename: str, results):
    for variant in SolverType:
        for optimisationTarget in OptimisationTarget:
            request_data = {
                "data": mps,
                "optimisationTarget": optimisationTarget.value,
                #Ignored, since this is used only for hash com. on FE
                "method": "REVISED",
                "version": "test",
                "solverConfiguration": solverConfiguration
            }             
            url = BE_URL + variant.value

            response = requests.post(url, json=request_data)
            if response.status_code == 200 and response.json()["solutionStatus"] == "SOLVED":
                results[variant.value].append({
                    "status": SolverResponse.OK,
                    "time": response.elapsed,
                    "file": filename,
                    "target": optimisationTarget.value
                })
            elif response.status_code != 200 and variant == SolverType.BASIC and "Problem contains G/E rows, that are not supported in basic simplex algorithm" in response.text:
                results[variant.value].append({
                    "status": SolverResponse.IGNORED,
                    "time": response.elapsed,
                    "file": filename,
                    "target": optimisationTarget.value
                })
            else:
                results[variant.value].append({
                    "status": SolverResponse.ERROR,
                    "time": response.elapsed,
                    "file": filename,
                    "target": optimisationTarget.value
                })


def run_tests(directory:str):
    #Init results as empty dictionary
    results = {}
    for variant in SolverType:
        results[variant.value] = []

    for root, dirs, files in os.walk(directory):
        for file in files:
            file_path = os.path.join(root, file)

            try:
                with open(file_path, "r", encoding="utf-8") as f:
                    print(file)
                    test_file_content = f.read()
                    call_solve_api_with_mps(test_file_content, file, results)
                    
            except Exception as e:
                print(f"Skipping {file_path}: {e}")
        
    return results

def get_version() -> str:
    response = requests.get("http://localhost:8080/api/simpler/health")
    if response.status_code != 200:
        raise Exception("Could not get health status response")
    return response.json()["version"]


def test_results(test_results):
    version = get_version()

    print("-------------------------------------")
    print("Performance test results of version: " + version)
    print("-------------------------------------")
    for variant in test_results:
        ms_total_for_variant = 0
        considered_for_average_calculation = 0
        print("-------- VARIANT " + variant + "---------------")

        for test_input_file_res in test_results[variant]:
            
            if test_input_file_res["status"] == SolverResponse.ERROR:
                print("Input " + test_input_file_res["file"] + " target " + test_input_file_res["target"] + " SOLVER_ERROR ❗")
                continue
            
            elif test_input_file_res["status"] == SolverResponse.IGNORED:
                print("Input " + test_input_file_res["file"] + " target " + test_input_file_res["target"] + " IGNORED ℹ️")
                continue

            request_duration_ms = test_input_file_res["time"].total_seconds() * 1000
            
            if request_duration_ms < EVALUATION_THRESHOLD_MS:
                print("Input " + test_input_file_res["file"] + " target " + test_input_file_res["target"] + " ✅")
                ms_total_for_variant += request_duration_ms
                considered_for_average_calculation += 1
            else:
                print("Input " + test_input_file_res["file"] + " target " + test_input_file_res["target"] + " TIME ELAPSED ⚠️")
                ms_total_for_variant += request_duration_ms
                considered_for_average_calculation += 1

        if considered_for_average_calculation == 0:
            print("Cannot calculate average solution time. 0 considered requests")
        else:
            print("Variant average solution time MS: " + str(ms_total_for_variant/considered_for_average_calculation))
        print("-------------------------------------")            
        
results = run_tests("./data")
test_results(results)