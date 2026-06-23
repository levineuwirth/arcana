//! Elvish Clancaller — `{G}{G}` 1/1 Creature — Elf Druid.
//!
//! Oracle:
//! * "Other Elves you control get +1/+1." — GAP (static anthem continuous
//!   effect; no trigger word, no activation cost).
//! * "{4}{G}{G}, {T}: Search your library for a card named Elvish Clancaller,
//!   put it onto the battlefield, then shuffle." — an activated ability with a
//!   mana + tap cost that tutors the named card straight onto the battlefield
//!   (shuffle is automatic). Searched by exact name via the `ObjectFilter.name`
//!   field looked up through the registry interner.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Elvish Clancaller");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "Other Elves you control get +1/+1." — continuous anthem with
    // no trigger/cost; not expressible in this card class.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}{G}{G}, {T}: Search your library for a card named Elvish Clancaller, put it onto the battlefield, then shuffle.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{G}{G}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_clancaller,
        }),
    )
}

fn tutor_clancaller(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let nm = reg.interner().lookup("Elvish Clancaller");
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        tapped: false,
    }]
}
