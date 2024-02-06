use stylus_sdk::prelude::sol_interface;
sol_interface! {
    interface IChildBitsave {
        function getSaving(string nameOfSaving) returns (bool);
    }
}
