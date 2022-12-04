#!/usr/bin/env python3

import argparse
from pathlib import Path
import struct
import imghdr

# PARSING = [
#     ("body", "body/bodies/*/universal/"),
#     ("head", "head/heads/*/universal/"),
#     ("ears", "head/ears/*/"),
#     ("arms_armour_plate", "arms/armour/plate/*/"),
#     ("arms_bracers", "arms/bracers/*/"),
#     ("bauldron", "bauldron/*/"),
# ]


def _png_size(image_path):
    with image_path.open("rb") as image_file:
        head = image_file.read(24)
        if len(head) != 24:
            raise Exception(f"File {image_path} is not a PNG file")
        what = imghdr.what(None, head)
        if what == "png":
            check = struct.unpack(">i", head[4:8])[0]
            if check != 0x0D0A1A0A:
                raise Exception(f"File {image_path} is not a PNG file")
            width, height = struct.unpack(">ii", head[16:24])
            return (width, height)

    raise Exception(f"File {image_path} is not a PNG file")


def _credit_content_as_dict(csv_content):
    dict_ = {}
    for line in csv_content.splitlines():
        dict_[line.split(",")[0]] = line
    return dict_


def _check_image_size(image_path, expected_image_width, expected_image_height):
    width, height = _png_size(image_path)
    if width != expected_image_width or height != expected_image_height:
        return False
    return True


def _explore(
    folder,
    image_paths,
    expected_image_width,
    expected_image_height,
):
    if "universal" in list(file.name for file in folder.iterdir()):
        for image_path in (folder / "universal").glob("*.png"):
            if _check_image_size(
                image_path,
                expected_image_width,
                expected_image_height,
            ):
                image_paths.append((image_path, folder / image_path.name))
            else:
                print(f"❕ Ignore image '{image_path}' : don't match with expected size")
    else:
        for file in folder.iterdir():
            if file.is_dir():
                _explore(
                    file,
                    image_paths,
                    expected_image_width,
                    expected_image_height,
                )
            elif file.is_file():
                if _check_image_size(
                    file,
                    expected_image_width,
                    expected_image_height,
                ):
                    image_paths.append((file, file))
                else:
                    print(f"❕ Ignore image '{file}' : don't match with expected size")


def import_(
    ulpcscg_spritesheets_path,
    lpcg_spritesheets_path,
    expected_image_width,
    expected_image_height,
):
    lpcg_credits_csv_file_path = Path("CREDITS.csv")
    lpcg_credits_csv_content = lpcg_credits_csv_file_path.read_text()
    lpcg_credits = set(lpcg_credits_csv_content.splitlines())
    ulpcscg_credits_csv_file_path = ulpcscg_spritesheets_path / ".." / "CREDITS.csv"
    ulpcscg_credits_csv_content = ulpcscg_credits_csv_file_path.read_text()
    ulpcscg_credits = _credit_content_as_dict(ulpcscg_credits_csv_content)

    image_paths = []
    _explore(
        ulpcscg_spritesheets_path,
        image_paths,
        expected_image_width,
        expected_image_height,
    )
    image_paths = [
        (
            image_path[0].relative_to(ulpcscg_spritesheets_path),
            image_path[1].relative_to(ulpcscg_spritesheets_path),
        )
        for image_path in image_paths
    ]
    for source, dest in image_paths:
        (lpcg_spritesheets_path / dest).parent.mkdir(exist_ok=True, parents=True)
        (lpcg_spritesheets_path / dest).write_bytes(
            (ulpcscg_spritesheets_path / source).read_bytes()
        )

        try:
            source_credit_line = ulpcscg_credits[str(source)]
        except KeyError:
            print(f"❗ Unable to find credit line for '{source}'")
        if source_credit_line not in lpcg_credits:
            lpcg_credits.add(source_credit_line)

    lpcg_credits_csv_file_path.write_text("\n".join(lpcg_credits))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--ulpcscg-spritesheets-path",
        default="../Universal-LPC-Spritesheet-Character-Generator/spritesheets",
    )
    parser.add_argument(
        "--lpcg-spritesheets-path",
        default="./spritesheets",
    )
    parser.add_argument(
        "--expected-image-width",
        type=int,
        default=832,
    )
    parser.add_argument(
        "--expected-image-height",
        type=int,
        default=1344,
    )
    args = parser.parse_args()

    import_(
        Path(args.ulpcscg_spritesheets_path),
        Path(args.lpcg_spritesheets_path),
        args.expected_image_width,
        args.expected_image_height,
    )
