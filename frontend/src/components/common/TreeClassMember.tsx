// import { useRecoilValue } from 'recoil';
// import { getUserByClass } from '../../recoil/selectors';
// import { Link } from 'react-router-dom';
// import { useMemo } from 'react';

// type TreeClassMemberProps = {
//     class_name?: string;
// }

// const TreeClassMember: React.FC<TreeClassMemberProps> = ( {class_name} ) => {
//     const member = useRecoilValue(getUserByClass(class_name));
//     // console.log(member);

//     const renderedMembers = useMemo(() => {
//         return member.map(user => {
//             return (
//                 <li key={user.id}>
//                     <Link to={`/user/${user.id}`} className="flex items-center gap-x-3.5 py-2 px-2.5 text-sm text-gray-700 rounded-lg hover:bg-gray-600 dark:bg-neutral-800 dark:text-neutral-400 dark:hover:text-neutral-300 dark:hover:bg-neutral-700">
//                     {user.name}
//                     </Link>
//                 </li>
//             )
//         });
//     }, [member]);

//     return(
//         <>
//             {
//                 renderedMembers
//             }
//         </>
//     )
// };
// export default TreeClassMember;