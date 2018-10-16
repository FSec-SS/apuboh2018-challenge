<!DOCTYPE html>
<head>
    <title>Can you see my secret?</title>
</head>
<body>
    <?php
    $db = mysqli_connect("localhost", "boh", "", "quote");
    if (ISSET($_GET["id"])) {
	$id = $_GET["id"];
	$sql = "SELECT * FROM `quote` INNER JOIN `user` ON `quote`.`user_id` = `user`.`user_id` WHERE `quote`.`user_id` = $id";
	$result = mysqli_query($db, $sql);
	$data = array();
	if (mysqli_num_rows($result) > 0) {
	    while ($row = mysqli_fetch_assoc($result)) {
		$data[] = $row;
	    }
	}
	$height = count($data);
	?>
        <table border="1">
    	<tr>
    	    <th>Username</th>
    	    <th>Quote</th>
    	</tr>
	    <?php
	    if ($height > 0) {
		for ($a = 0; $a < $height; $a++) {
		    ?>
	    	<tr>
	    	    <td><?php echo $data[$a]["username"] ?></td>
	    	    <td><?php echo $data[$a]["quote"] ?></td>
	    	</tr>
		<?php }
	    } else {
		?>
		<tr>
		    <td>Invalid Input</td>
		    <td>Invalid Input</td>
		</tr>	
	<?php } ?>
        </table>
<?php } ?>

    <form action='<?php echo $_SERVER['PHP_SELF'];?>' method='GET'>
	<select name='id'>
	    <?php
	    $sql = "SELECT * FROM `user`";
	    $result = mysqli_query($db, $sql);
	    $data = array();
	    if (mysqli_num_rows($result) > 0) {
		while ($row = mysqli_fetch_assoc($result)) {
		    $data[] = $row;
		}
	    }
	    $height = count($data);
	    for ($a = 0; $a < $height; $a++) {
		?>
    	    <option value='<?php echo $data[$a]["user_id"] ?>'><?php echo $data[$a]["username"] ?></option>
<?php } ?>
	</select>
	<button type='SUBMIT'>SEE QUOTE</button>
    </form>
</body>
</html>