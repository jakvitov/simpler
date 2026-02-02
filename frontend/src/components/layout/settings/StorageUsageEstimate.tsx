import {useEffect, useState} from "react";
import {Container, Table} from "react-bootstrap";
import {clear} from "idb-keyval";

function StorageUsageEstimate() {

    const [storageEstimate, setStorageEstimate] = useState<StorageEstimate|null>(null)

    useEffect(() => {
        navigator.storage.estimate().then(setStorageEstimate)
    }, [storageEstimate])


    const handleCleanSorage = () => {
        console.log("Cleaning all persisted data. ")
        clear().then(() => console.log("Persisted data cleaned."))
        navigator.storage.estimate().then(setStorageEstimate)
    }

    if (storageEstimate === null) {
    } else {
        return <Container className={"mt-2"}>
            <h3>Storage</h3>
            Simpler uses storage in order to persist problem data and solutions for later displays. This data does not usually get
            cleaned up automatically. You can manually inspect all the data in your browsers <a href={"https://en.wikipedia.org/wiki/IndexedDB"}>Indexed DB</a>.
            Note that the usage will never drop to 0 since other data than this are always persisted in the browser.
            <Table bordered className={"mt-2"}>
                <thead>
                <tr>
                    <th>Storage usage estimate</th>
                    <th>Storage used</th>
                    <th>Available storage</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td><a onClick={handleCleanSorage} href={"#"}> Clean storage</a></td>
                    <td>{storageEstimate.usage === undefined ? "Could not obtain.": storageEstimate.usage/1000} KB</td>
                    <td>{storageEstimate.quota === undefined ? "Could not obtain. ": storageEstimate.quota/1000} KB</td>
                </tr>
                </tbody>
            </Table>
        </Container>
    }

}

export default StorageUsageEstimate