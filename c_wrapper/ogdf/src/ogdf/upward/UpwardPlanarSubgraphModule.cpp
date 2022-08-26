/** \file
 * \brief Implementation of class UpwardPlanarSubgraphModule
 *
 * \author Carsten Gutwenger
 *
 * \par License:
 * This file is part of the Open Graph Drawing Framework (OGDF).
 *
 * \par
 * Copyright (C)<br>
 * See README.md in the OGDF root directory for details.
 *
 * \par
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * Version 2 or 3 as published by the Free Software Foundation;
 * see the file LICENSE.txt included in the packaging of this file
 * for details.
 *
 * \par
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * \par
 * You should have received a copy of the GNU General Public
 * License along with this program; if not, see
 * http://www.gnu.org/copyleft/gpl.html
 */

#include <ogdf/upward/UpwardPlanarSubgraphModule.h>

namespace ogdf {


void UpwardPlanarSubgraphModule::callAndDelete(
	GraphCopy &GC,
	List<edge> &delOrigEdges)
{
	List<edge> delEdges;

	call(GC, delEdges);

	ListConstIterator<edge> it;
	for(it = delEdges.begin(); it.valid(); ++it) {
		edge eCopy = *it;

		delOrigEdges.pushBack(GC.original(eCopy));
		GC.delEdge(eCopy);
	}
}

}
