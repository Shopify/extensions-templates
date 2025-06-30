import type {
  CartValidationsGenerateRunInput,
  CartValidationsGenerateRunResult,
  FunctionError,
} from "../generated/api";

export function cart_validations_generate_run(input: CartValidationsGenerateRunInput): CartValidationsGenerateRunResult {
  const errors: FunctionError[] = input.cart.lines
    .filter(({ quantity }) => quantity > 1)
    .map(() => ({
      localizedMessage: "Not possible to order more than one of each",
      target: "$.cart",
    }));

  return {
    errors
  }
};