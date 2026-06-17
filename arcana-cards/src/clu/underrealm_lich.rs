//! Underrealm Lich — `{3}{B}{G}` 4/3 Zombie Elf Shaman.
//! Draw-replacement (look at top three, one to hand, rest to graveyard).
//! Pay 4 life: this creature gains indestructible until end of turn. Tap it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Underrealm Lich");
    let zombie = reg.interner_mut().intern("Zombie");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP: "If you would draw a card, instead look at the top three cards..." — a draw-
    // replacement static effect; the demonstrated surface has no replacement-of-draw hook
    // for a creature's printed text.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Pay 4 life: This creature gains indestructible until end of turn. Tap it.".into(),
                cost: ActivationCost {
                    life: 4,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_indestructible_tap,
            }),
    )
}

fn gain_indestructible_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::Tap { target: ctx.source },
    ]
}
