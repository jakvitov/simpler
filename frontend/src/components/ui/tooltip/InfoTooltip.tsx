import "./InfoTooltip.css"

type InfoTooltipProps = {
    text: string
}

export default function InfoTooltip(props: InfoTooltipProps) {
    return (
        <span className="info-tooltip">
      <span className="info-icon" aria-hidden>i</span>
      <span className="tooltip" role="tooltip">
        {props.text}
      </span>
    </span>
    );
}