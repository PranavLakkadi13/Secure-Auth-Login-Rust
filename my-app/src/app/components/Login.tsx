import { ChangeEvent, FormEvent, useState } from "react";
import styles from "./Login.module.css"
import { router } from "next/client";
import axios from "axios";


interface LoginForm {
    email: string; 
    password: string
}

const Login: React.FC = () => {
        const [email, setEmail] = useState<string>('');
        const [password, setPassword] = useState<string>('');
    
        const handleInputs = (e: ChangeEvent<HTMLInputElement>) => {
            const { id, value } = e.target;
            if (id === "email") setEmail(value);
            if (id === "password") setPassword(value);
        }
    
        const handleSubmit = async (e: FormEvent) => {
        e.preventDefault();
        const form: LoginForm = { email, password }
        
        try {
            const response = await axios.post("http://localhost:9000/api/login", form, {
                headers: {
                    "Content-Type": "application/json"
                },
                withCredentials: true
            });

            if (response.status === 200) {
                await router.push("http://localhost:3001/profile");
            }
            else {
                console.error("Request failed to login");
            }
        } catch (err) {
            console.log("failed ", err);
        }
    }
    
    return (
        <div className={styles.container}>
            <form onSubmit={handleSubmit} className={styles.form}>

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
                <button type='submit' className={ styles.button}>Login </button>
            </form>
            <div className={styles.LinkContainer}> <a href="/register" className={ styles.link}>Create Account</a></div>
        </div>
    );
}


export default Login;