import subprocess
import unittest

class LRUCacheE2ETests(unittest.TestCase):

    def test_put_and_get(self):
        # Assuming the compiled binary is located at ./target/release/lru_cache_rs
        process = subprocess.Popen(['cargo', 'run', '--release'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        
        # Put a key-value pair
        process.stdin.write("put key1 value1\n")
        process.stdin.flush()
        
        # Get the value
        process.stdin.write("get key1\n")
        process.stdin.flush()
        
        # Exit the process
        process.stdin.write("exit\n")
        process.stdin.close()

        stdout, stderr = process.communicate()
        return_code = process.returncode

        self.assertEqual(return_code, 0)

        # Check the output
        self.assertIn("value1", stdout)

    def test_eviction(self):
        # Assuming the compiled binary is located at ./target/release/lru_cache_rs
        process = subprocess.Popen(['cargo', 'run', '--release'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        
        # Set capacity to 2 (assuming this is configurable, if not, adjust the test)
        # Note: This example assumes you have a way to set the capacity.  If not, you'll need to adjust the test.
        process.stdin.write("capacity 2\n")
        process.stdin.flush()

        # Put three key-value pairs to trigger eviction
        process.stdin.write("put key1 value1\n")
        process.stdin.flush()
        process.stdin.write("put key2 value2\n")
        process.stdin.flush()
        process.stdin.write("put key3 value3\n")
        process.stdin.flush()
        
        # Try to get the first key - should be evicted
        process.stdin.write("get key1\n")
        process.stdin.flush()

        # Exit the process
        process.stdin.write("exit\n")
        process.stdin.close()

        stdout, stderr = process.communicate()
        return_code = process.returncode

        self.assertEqual(return_code, 0)

        # Check the output - key1 should not be found
        self.assertNotIn("value1", stdout)
        self.assertIn("None", stdout)  # Assuming None is returned when not found


