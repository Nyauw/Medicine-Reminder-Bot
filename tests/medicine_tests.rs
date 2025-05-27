#[cfg(test)]
mod tests {
    use medicine_reminder::Medicine;
    use chrono::NaiveTime;

    #[test]
    fn test_take_dose_with_amount() {
        let mut medicine = Medicine::new(
            "Test Medicine".to_string(),
            10,
            vec![NaiveTime::from_hms_opt(8, 0, 0).unwrap()],
        );

        // Test taking 1 dose
        assert!(medicine.take_dose(1));
        assert_eq!(medicine.quantity, 9);

        // Test taking 3 doses
        assert!(medicine.take_dose(3));
        assert_eq!(medicine.quantity, 6);

        // Test taking more than available
        assert!(!medicine.take_dose(10));
        assert_eq!(medicine.quantity, 6); // Should remain unchanged

        // Test taking exact remaining amount
        assert!(medicine.take_dose(6));
        assert_eq!(medicine.quantity, 0);

        // Test taking from empty stock
        assert!(!medicine.take_dose(1));
        assert_eq!(medicine.quantity, 0);
    }

    #[test]
    fn test_add_quantity() {
        let mut medicine = Medicine::new(
            "Test Medicine".to_string(),
            5,
            vec![NaiveTime::from_hms_opt(8, 0, 0).unwrap()],
        );

        medicine.add_quantity(10);
        assert_eq!(medicine.quantity, 15);

        medicine.add_quantity(25);
        assert_eq!(medicine.quantity, 40);
    }

    #[test]
    fn test_medicine_creation() {
        let times = vec![
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
        ];
        
        let medicine = Medicine::new(
            "Vitamin C".to_string(),
            30,
            times.clone(),
        );

        assert_eq!(medicine.name, "Vitamin C");
        assert_eq!(medicine.quantity, 30);
        assert_eq!(medicine.reminder_times, times);
        assert!(medicine.is_active);
    }
}
