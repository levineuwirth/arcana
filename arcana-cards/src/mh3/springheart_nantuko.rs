//! Springheart Nantuko — `{1}{G}` 1/1 green Enchantment Creature — Insect Monk.
//! Bestow {1}{G} (enchanted creature gets +1/+1).
//! Landfall — Whenever a land you control enters, you may pay {1}{G} if this
//! permanent is attached to a creature you control. If you do, create a token
//! that's a copy of that creature. If you didn't create a token this way,
//! create a 1/1 green Insect creature token.
//!
//! GAP: Bestow (and its "+1/+1 to enchanted creature" static) is not in the
//! demonstrated keyword surface, and Landfall is not a keyword variant —
//! both are noted and omitted from `keywords`.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Springheart Nantuko");
    let insect = reg.interner_mut().intern("Insect");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent()
                    .with_types(TypeLine::LAND.into())
                    .controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: landfall,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn landfall(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "you may pay {1}{G} if attached to a creature you control → create
    // a token that's a copy of that creature" branch is not expressible — the
    // host creature this Aura/Bestow permanent is attached to is not exposed as a
    // resolution-time id, so the conditional copy-of-host cannot be built. We
    // emit the unconditional fallback (the 1/1 green Insect creature token).
    let insect = reg.interner().lookup("Insect").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: insect,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
