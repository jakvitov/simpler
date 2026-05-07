import {Container} from "react-bootstrap";
import {InlineMath} from "react-katex";
import {Link} from "react-router-dom";

function IntroManual() {
    return (
    <Container className={"pt-5 pb-5"}>
        <h2>How to use Simpler?</h2>
        <p>Simpler <i>(Simplex solver)</i> offers two main functionalities:</p>
        <ul>
            <li>Verification of MPS input and its parsing to LP equations</li>
            <li>Interactive solution of linear optimizations with different algorithmic variants</li>
        </ul>
        <p>Based on your use-case choose either <Link to={"/verify-mps"}>Verify MPS</Link> or <Link to={"solve-lp"}>Solve LP</Link>, enter your problem and hit submit. You can tweak all solver configurations in the <Link to={"/settings"}>Settings</Link> section.</p>


        <h3 className={"mt-4"}>Input formats</h3>
        <h4 className={"mt-4"}>Interactive input</h4>
        <p><Link to={"/solve-lp/interactive"} >Interactive input</Link> offers simplified and user friendly way of entering linear optimisation problems. Just enter variable coefficients into the given table and hit the submit button. Resizing of the input table can be achieved by clicking the <i>+ Variable</i>, <i>+ Constraint</i> and <i>-</i> buttons. Accepted number formats are similar across MPS and interactive input.</p>
        <h4 className={"mt-4"}>MPS</h4>
        <p>Main input format for Simpler is <a href={"https://lpsolve.sourceforge.net/5.5/mps-format.htm"}>MPS</a>. Original MPS format is quite restrictive, therefore for the convenience of users Simpler ignores all row/column character restrictions and relies solely on keywords, unknown keywords and sections are skipped and mostly do not trigger and error.</p>
        <h4 className={"mt-4"}>Numbers</h4>
        <p>Simpler works mainly with rational numbers and displays the results as whole numbers or fractions. In all input forms numbers can be entered as:</p>
        <ul>
            <li>Plain whole numbers like: <code>1</code> or <code>-4</code></li>
            <li>Rational numbers like: <code>1/50</code> or <code>-5/99</code></li>
            <li>Floating point numbers with delimiter <code>.</code> like: <code>1.5</code></li>
        </ul>
        <p>No other number input formats are supported, for example <code>-0,55746</code> will trigger an error.</p>

        <h4 className={"mt-4"}>Comments</h4>
        <p>Comments in MPS inputs are supported and are started with the <code>#</code> character. Everything between the first <code>#</code> character and the end of the line is ignored. For example line: <code>x_1        OBJ   #  1\n</code> will be processed as <code>x_1        OBJ</code>.</p>

        <h4 className={"mt-4"}>LaTeX support</h4>
        <p>Simpler uses LaTeX formatting in all non-error outputs. This means that entering variable name like <code>x_1</code> will result in it being rendered as: <InlineMath math={"x_1"} />. Note, that this applies to all nameable components of the MPS input format (for example RHS names as well).</p>

        <p><i>Example input MPS:</i></p>
        <div
            style={{
                width: '100%',
                backgroundColor: '#F5F5F5',
                color: 'black',
                fontSize: '1rem',
                fontFamily: 'monospace',
                lineHeight: '1.5',
                border: '0px',
                padding: '0.375rem 0.75rem',
                whiteSpace: 'pre-wrap',   // ✅ preserves \n
                overflowY: 'auto',        // optional: scroll like textarea
            }}
        >   {"#Simple LP example \n#Simpler compatible MPS\nNAME          ExampleMps\nROWS\nN  OBJ\nL  C1\nL  C2\nCOLUMNS\nx_1        OBJ     1\nx_1        C1      1\nx_1        C2      0\nx_2        OBJ     1\nx_2        C1      0\nx_2        C2      1\nRHS\nRHS1      C1      2\nRHS1      C2      2\nENDATA"}</div>

        <h3 className={"mt-4"}>Solver variants</h3>
        <p>Simpler offers detailed solution output for different algorithmic variants of the Simplex algorithm. You can choose your desired solver variant in the <Link to={"/solve-lp"}>Solve LP</Link> section. Currently supported variants are:</p>

        <ul>
            <li>Basic simplex (primary simplex algorithm, with tableau solution)</li>
            <li>Two phase simplex (two-phase simplex algorithm, with tableau solution)</li>
            <li>Revised simplex algorithm (two-phase simplex algorithm, solved using the revised method)</li>
            <li>Multiplicative simplex algorithm (two phase simplex algorithm, edited version of the revised method with basis matrix inverse calculation optimisations)</li>
        </ul>

        <h3 className={"mt-4"}>Troubleshooting</h3>
        <p>Simpler does cache backend inputs and outputs in the frontend indexed DB, this may occasionally cause minor problems. First step in troubleshooting is clicking the "Clean storage" button on the
        bottom of your screen. If that doesn't help and problem persists, try restarting Simpler, opening it in different browser or in private mode. Try and keep your application updated by downloading the newest version when it comes out on <Link to={"https://github.com/jakvitov/simpler/releases"}>GitHub</Link>.</p>
        <h3 className={"mt-4"}>Bug reporting</h3>
        <p>Reporting bugs helps Simpler to advance and provide better user experience. If any error occurs while using the application, report bug link will appear. If you click it, it will either contain prefilled
            information about application caught error, or ask you to describe your problem if none was caught. In case of prefilled bugreport, you get full overview of what is being sent, so that you can personally check for sensitive information.</p>
        <p>Try to always include your Simpler version and input, that triggered the bug, so that it can be tracked down and repaired swiftly.</p>
    </Container>
    )
}

export default IntroManual