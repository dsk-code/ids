import { Center, Loader, Stack } from '@mantine/core'
import React from 'react'

export const PageLoader: React.FC = () => {
  return (
    <Center>
        <Stack h={500} justify="center">
            <Loader color="blue" size="sm"/>
        </Stack>       
    </Center>
  )
}