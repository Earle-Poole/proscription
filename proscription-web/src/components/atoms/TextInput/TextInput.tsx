import classNames from 'classnames'
import { FC } from 'react'
import { FieldValues, UseFormRegister } from 'react-hook-form'

type ReactHookFormProps =
  | {
      register: UseFormRegister<FieldValues>
      value: string
      isRequired?: boolean
    }
  | {
      register?: never
      value?: never
      isRequired?: never
    }

type TextInputProps = {
  label?: string
  className?: string
  register?: UseFormRegister<FieldValues>
} & ReactHookFormProps

/**
 * The TextInput component is a simple text input field that
 * conditionally uses react-hook-form to register the input.
 *
 * @property label - The label for the input
 * @property className - (Optional) The class name for the input
 * @property register - (Optional) The register function from react-hook-form
 * @property value - (Optional) The value for the input
 * @returns
 */
const TextInput: FC<TextInputProps> = ({
  label,
  className,
  register,
  isRequired,
  value,
}) => {
  return (
    <div className={classNames(className, 'flex flex-col')}>
      {label && (
        <label>
          {label}
          {isRequired && <span className='text-red-600'>*</span>}
        </label>
      )}
      <input
        className='bg-indigo-200 border-2 border-indigo-300 rounded-md p-2'
        {...(register
          ? register(value, { required: isRequired })
          : {})}></input>
    </div>
  )
}

export default TextInput
