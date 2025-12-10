import { ChangeEvent, FormEvent, useState } from 'react';
import styles from './Register.module.css';
import axios from 'axios';
import {router} from 'next/client';


interface RegisterForm {
    name: string;
    email: string;
    password: string 
}


const Register: React.FC = () => {
    const [name, setName] = useState<string>('');
    const [email, setEmail] = useState<string>('');
    const [password, setPassword] = useState<string>('');

    const handleInputs = (e: ChangeEvent<HTMLInputElement>) => {
        const { id, value } = e.target;
        if (id === "name") setName(value);
        if (id === "email") setEmail(value);
        if (id === "password") setPassword(value);
    }

    const handleSubmit = async (e: FormEvent) => {
        e.preventDefault();
        const form: RegisterForm = { name, email, password }
        
        try {
            const response = await axios.post("http://localhost:9000/api/register", form, {
                headers: {
                    "Content-Type": "application/json"
                },
            });

            if (response.status === 200) {
                await router.push("http://localhost:3001/");
            }
            else {
                console.error("Request failed to register");
            }
        } catch (err) {
            console.log("failed ", err);
        }
    }

    return (
        <div className={styles.container}>
            <form onSubmit={handleSubmit} className={styles.form}>
                <div className={styles.field}>
                    <label htmlFor="name" className={styles.label}>
                        Name:
                    </label>
                    <input type="text" id='name' className={ styles.input} value={name} onChange={handleInputs} />
                </div>

                <div className={styles.field}>
                    <label htmlFor="email" className={styles.label}>
                        Email:
                    </label>
                    <input type="email" id='email' className={ styles.input} value={email} onChange={handleInputs} />
                </div>

                <div className={styles.field}>
                    <label htmlFor="password" className={styles.label}>
                        Password:
                    </label>
                    <input type="password" id='password' className={ styles.input} value={password} onChange={handleInputs} />
                </div>
                <button type='submit' className={ styles.button}>Register</button>
            </form>
            <div className={styles.LinkContainer}> <a href="/" className={ styles.link}> Already have an account</a></div>
        </div>
    );
}

export default Register;