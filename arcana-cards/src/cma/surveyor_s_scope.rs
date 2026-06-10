//! Surveyor's Scope — `{2}` artifact.
//! "{T}, Exile this artifact: Search your library for up to X basic
//! land cards, where X is the number of players who control at least
//! two more lands than you. Put those cards onto the battlefield, then
//! shuffle." X is computed at resolution; the "up to" decline choice
//! is a GAP (modeled as exactly X searches).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surveyor's Scope");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Exile this artifact: Search your library for up \
                       to X basic land cards, where X is the number of \
                       players who control at least two more lands than \
                       you. Put those cards onto the battlefield, then \
                       shuffle."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: fetch_basics,
            },
        ),
    )
}

fn fetch_basics(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let land_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    let mine = script::count_matching(state, &land_filter, ctx.controller);
    let mut x = 0u32;
    for p in script::all_players(state) {
        if p == ctx.controller {
            continue;
        }
        if script::count_matching(state, &land_filter, p) >= mine + 2 {
            x += 1;
        }
    }
    // GAP: "up to X" — the search is modeled as exactly X (no per-card
    // decline choice).
    (0..x)
        .map(|_| Effect::TutorToBattlefield {
            player: ctx.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
            tapped: false,
        })
        .collect()
}
