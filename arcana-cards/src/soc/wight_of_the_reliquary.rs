//! Wight of the Reliquary — `{B}{G}` 2/2 Zombie Knight with Vigilance.
//! "This creature gets +1/+1 for each creature card in your graveyard."
//! "{T}, Sacrifice another creature: Search your library for a land card,
//! put it onto the battlefield tapped, then shuffle."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wight of the Reliquary");
    let zombie = reg.interner_mut().intern("Zombie");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "This creature gets +1/+1 for each creature card in your
    // graveyard." — a pure self-buffing continuous static (no trigger / no
    // cost) is not expressible with the available primitives.

    let creature_filter = ObjectFilter {
        types: Some(TypeLine::CREATURE.into()),
        ..ObjectFilter::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice another creature: Search your library for a land card, put it onto the battlefield tapped, then shuffle.".into(),
            cost: ActivationCost {
                tap: true,
                sacrifice_other: Some(creature_filter),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_land_tapped,
        }),
    )
}

fn tutor_land_tapped(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        tapped: true,
    }]
}
