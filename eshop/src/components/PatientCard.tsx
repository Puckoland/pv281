import {IProduct} from "./PatientsList";

interface IPatientCard {
    index: number;
    patient: IProduct;
}

export const PatientCard = ({ index, patient }: IPatientCard) => {
    return (
        <div className="ml-3 flex flex-col justify-between text-left">
            <span className="text-md font-medium text-gray-800"><strong>Name</strong>: {patient.name} <strong>Price</strong>: {patient.price}</span>
        </div>
    )
}