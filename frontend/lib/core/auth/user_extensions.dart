import 'auth_models.dart';

extension UserExtensions on User {
  String get displayName {
    if (firstName.isNotEmpty && lastName.isNotEmpty) {
      return '$firstName $lastName';
    }
    if (firstName.isNotEmpty) return firstName;
    if (lastName.isNotEmpty) return lastName;
    return email.split('@').first;
  }

  String get initials {
    if (firstName.isNotEmpty && lastName.isNotEmpty) {
      return '${firstName[0]}${lastName[0]}'.toUpperCase();
    }
    if (firstName.isNotEmpty) {
      return firstName[0].toUpperCase();
    }
    if (lastName.isNotEmpty) {
      return lastName[0].toUpperCase();
    }
    return email[0].toUpperCase();
  }
}
