import TextInput from '@atoms/TextInput'
import { API_URL } from '@constants'
import MainLayout from '@layouts/MainLayout'
import classNames from 'classnames'
import { FieldValues, useForm } from 'react-hook-form'

enum MainMenuInputValues {
  ROOM_CODE = 'room_code',
  NAME = 'name',
}

const MainMenu = () => {
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm()

  const onJoinSubmit = (data: FieldValues) =>
    fetch('http://' + API_URL + '/join', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    })
  const onCreateSubmit = (data: FieldValues) =>
    fetch('http://' + API_URL + '/create')
  return (
    <MainLayout>
      <main className='absolute bottom-0 left-0 right-0 top-[4.5rem] flex justify-center p-10 bg-indigo-200'>
        <section className='h-fit border-2 border-indigo-600 bg-slate-50 text-black text-2xl font- rounded-lg p-4 shadow-xl'>
          <form
            className='p-2 flex flex-col'
            onSubmit={handleSubmit(onJoinSubmit)}>
            <TextInput
              register={register}
              isRequired
              className={classNames({
                'mb-6': !errors[MainMenuInputValues.ROOM_CODE],
              })}
              label='Room Code'
              value={MainMenuInputValues.ROOM_CODE}
            />
            {errors[MainMenuInputValues.ROOM_CODE] && (
              <span className='text-red-600 text-sm mx-2 mb-1'>
                This field is required
              </span>
            )}
            <TextInput
              register={register}
              className='mb-8'
              label='Name'
              value={MainMenuInputValues.NAME}
            />
            <button
              className='text-white bg-indigo-600 font-extrabold p-2 mb-8'
              type='submit'>
              JOIN
            </button>
            <button
              className='text-white bg-indigo-600 font-extrabold p-2'
              onClick={handleSubmit(onCreateSubmit)}>
              CREATE
            </button>
          </form>
        </section>
      </main>
    </MainLayout>
  )
}

export default MainMenu
