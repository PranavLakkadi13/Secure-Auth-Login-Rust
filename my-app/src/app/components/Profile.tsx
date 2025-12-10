import { useState } from "react";
import styles from "./Profile.module.css";

interface ProfileDetails {
    name: string;
    email: string;
}

const Profile: React.FC = () => {
    const [profile, setProfile] = useState<ProfileDetails | null>(null);
    
    return (
        <div className={styles.profileContainer}>
            {
                profile? (
                    <div>
                        <h1> Profile </h1>
                        <p>Name: { profile.name}</p>
                        
                    </div>
                )
            }
        </div>
    )
}

export default Profile;