import {invoke} from "@tauri-apps/api/tauri";
import {useEffect, useState} from "react";
import {PatientCard} from "./PatientCard";

export interface IProduct {
    name: string;
    price: number;
}

export const PatientsList = () => {
    const [patients, setPatients] = useState<IProduct[]>([]);

    useEffect(() => {
        const getPatients = async () => {
            const patients: IProduct[] = await invoke('get_products');
            setPatients(patients);
        }
        getPatients();
    }, [])

    return (
        <div className="flex flex-col w-3/4 h-full text-center">
            <h1 className="w-full font-semibold mt-4 text-2xl pb-4 border-b-2 border-blue-100">Products</h1>
            {patients?.map(
                (patient: IProduct, idx: number) => <PatientCard index={idx} patient={patient} />)
            }
        </div>
    )
}