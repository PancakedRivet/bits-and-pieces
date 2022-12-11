# Puzzle 7: https://adventofcode.com/2022/day/7

# Open the input file
with open('input.txt') as file:
    lines = file.readlines()


class Folder:
    # Create a class to hold information about a given folder

    def __init__(self, parent, name, depth, line_responsible):
        self.parent_folder = parent
        self.name = name
        self.total_size = 0
        self.child_files = []
        self.child_folders = []
        self.depth = depth
        self.line_responsible = line_responsible

    def calculate_size(self, debug=False):
        if debug:
            print("START %s.calculate_size()" % self.name)

        child_file_size = 0
        child_folder_size = 0

        for child in self.child_files:
            calc_size = int(child.size)
            child_file_size += calc_size
            if debug:
                print("child_files %s.calculate_size() = %s" %
                      (child.name, calc_size))

        if debug:
            print("child_files total = %s" % child_file_size)

        for child in self.child_folders:
            child.calculate_size(debug=debug)
            child_folder_size += child.total_size
            if debug:
                print("child_folders %s.calculate_size() = %s" %
                      (child.name, child.total_size))

        if debug:
            print("child_folders total = %s" % child_folder_size)

        total_size = child_file_size + child_folder_size

        if debug:
            print("END %s.calculate_size() = %s" % (self.name, total_size))

        self.total_size = total_size

    def new_folder(self, folder, debug=False):
        if debug:
            print("Adding folder: %s" % folder.name)
        self.child_folders.append(folder)
        if debug:
            self.list_child_folders()
        self.calculate_size()

    def new_file(self, file, debug=False):
        if debug:
            print("Adding file: %s" % file.name)
        self.child_files.append(file)
        if debug:
            print("%s.child_files: %s" % (self.name, self.child_files))
        self.calculate_size()

    def list_folder(self, debug=False):
        if debug:
            print("DEBUG: Inside folder: %s" % self.name)

        depth_factor = self.depth + 1
        for child_file in self.child_files:
            print("%sFILE: %s SIZE: %s" %
                  ("-" * depth_factor, child_file.name, child_file.size))
        for child_folder in self.child_folders:
            print("%sFOLDER: %s SIZE: %s" %
                  ("-" * depth_factor, child_folder.name, child_folder.total_size))
            # child_folder.list_folder(debug=True)
            child_folder.list_folder(debug=False)

    def list_child_folders(self):
        message = "%s.child_folders = " % self.name
        if len(self.child_folders) == 0:
            message += "NONE"
        else:
            for child_folder in self.child_folders:
                message += child_folder.name + ", "
        print(message)

    def __str__(self):
        return self.name

    def __repr__(self):
        message = ""
        for child in self.child_folders:
            message += child.name + ", "
        return message


class File:
    # Create a class to hold information about a given file
    def __init__(self, name, size):
        self.name = name
        self.size = size


def item_exists(item_name, collection, debug=False):
    # Check if an item exists in a given collection
    item_exists = False
    if debug:
        print("testing item %s in [%s]" % (item_name, collection))

    for child in collection:

        if debug:
            print(child.name)

        if child.name == item_name:
            item_exists = True
            break

    if debug:
        print("testing item %s - result = %s" % (item_name, item_exists))

    return item_exists


def find_folder(folder_name, collection, debug=False):
    # Assume if a given folder exists and return it or raise and exception
    if debug:
        print("finding folder %s in [%s]" % (folder_name, collection))

    for child in collection:
        if debug:
            print(child.name)
        if child.name == folder_name:
            if debug:
                print("finding folder %s - result found" % folder_name)
            return child

    message = "ERROR: finding item %s - result NOT found" % folder_name
    raise Exception(message)


working_directory_chain = []
current_command = ""
current_folder = Folder(None, "/", -1, "$ cd /")
folder_list = [current_folder]

for line in lines:
    # Remove the newline character from the line
    line_stripped = line.replace("\n", "")
    # Split the line by the comma
    line_split = line_stripped.split()

    # Debugging statements
    # print("SHELL MESSAGE =  %s" % line_stripped)
    # print("Current Folder =  %s" % current_folder.name)
    # print("Child folders = %s" % current_folder.child_folders)
    # current_folder.list_child_folders()

    command_issued = False
    if line_split[0] == "$":
        command_issued = True
        current_command = line_split

    if command_issued:

        if line_split[1] == "cd":
            # Changing directory

            if line_stripped == "$ cd /":
                continue

            if line_split[2] != "..":
                # cd <foldername>
                working_directory_chain.append(line_split[2])
                folder_name = line_split[2]

                folder_exists = item_exists(
                    folder_name, current_folder.child_folders)

                if not folder_exists:
                    new_folder = Folder(current_folder, folder_name,
                                        len(working_directory_chain) - 1, line_stripped)
                    folder_list.append(new_folder)
                    current_folder.new_folder(new_folder, debug=False)
                    current_folder = new_folder
                else:
                    current_folder = find_folder(
                        folder_name, current_folder.child_folders)

            else:
                # cd ..
                working_directory_chain.pop()
                current_folder = current_folder.parent_folder

        if line_split[1] == "ls":
            # Listing directory contents
            continue

    else:
        if line_split[0] == "dir":
            # Listed directory
            folder_name = line_split[1]

            folder_exists = item_exists(
                folder_name, current_folder.child_folders)

            if not folder_exists:
                new_folder = Folder(current_folder, folder_name,
                                    current_folder.depth + 1, line_stripped)
                folder_list.append(new_folder)
                current_folder.new_folder(new_folder, debug=False)

        else:
            # Listed file
            file_name = line_split[1]
            file_size = int(line_split[0])
            new_file = File(file_name, file_size)

            file_exists = False
            for child_file in current_folder.child_files:
                if child_file.name == file_name:
                    # print("File Exists: %s" % child_file.name)
                    file_exists = True
                    break
            if not file_exists:
                # print("New File")
                current_folder.new_file(new_file)

    # Recalculate the folder size with any new information
    current_folder.calculate_size()


root_folder = folder_list[0]
# root_folder.list_folder()
root_folder.calculate_size(debug=False)

# Calculate the size of directories under 100000 size
directory_total = 0
for folder in folder_list:
    if folder.total_size <= 100000:
        # print("PASS folder: %s size: %s" % (folder, folder.total_size))
        directory_total += folder.total_size
    else:
        # print("FAIL folder: %s size: %s" % (folder, folder.total_size))
        pass

print("Total sum of directories with size <= 100000 = %s" % directory_total)
# print(len(folder_list))

# Puzzle 7: Part 2: https://adventofcode.com/2022/day/7#part2

root_folder = folder_list[0]
# root_folder.list_folder()
root_folder.calculate_size()

total_space = 70000000
update_space_required = 30000000

# Taken from the debug results of Part 1
space_taken = 47052440

remaining_space_needed = abs(total_space - update_space_required - 47052440)

smallest_folder = None
smallest_folder_size = total_space
for folder in folder_list:
    if folder.total_size >= remaining_space_needed and folder.total_size < smallest_folder_size:
        smallest_folder = folder
        smallest_folder_size = folder.total_size

print("Size of smallest directory to free enough space = %s" %
      smallest_folder_size)
