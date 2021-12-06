import {invoke} from "@tauri-apps/api/tauri";
import { ChangeEvent, FormEventHandler, useState } from "react";

export const PatientForm = () => {
    const [name, setName] = useState("");
    const [price, setPrice] = useState(0);

    const handleNameChange = (event: ChangeEvent<HTMLInputElement>) => {
        setName(event.target.value);
    }

    const handlePriceChange = (event: ChangeEvent<HTMLInputElement>) => {
        setPrice(+event.target.value);
    }

    const handleSubmit = async (event: any) => {
        await invoke('add_product', {
            name,
            price,
        });
    }

    return (
        <form onSubmit={handleSubmit}>
            <input type="text" id="name" name="name" value={name} onChange={handleNameChange} />
            <input type="number" id="price" name="price" value={price} onChange={handlePriceChange} />
            <input type="submit" value="Submit" />
        </form>
    )
}