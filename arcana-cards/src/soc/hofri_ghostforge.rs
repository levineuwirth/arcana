//! Hofri Ghostforge — `{3}{R}{W}` 4/5 Legendary Dwarf Cleric.
//!
//! * "Spirits you control get +1/+1 and have trample and haste." — a
//!   static anthem; no trigger word, no cost, no Effect primitive.
//!   GAP: tribal anthem static.
//! * "Whenever another nontoken creature you control dies, exile it. If
//!   you do, create a token that's a copy of that creature, except it's a
//!   Spirit in addition to its other types and it has 'When this token
//!   leaves the battlefield, return the exiled card to its owner's
//!   graveyard.'" — a dies trigger on another nontoken creature you
//!   control. Implemented as: mint a token copy of the dying creature
//!   (`CopyPermanent`), then exile the original card from the graveyard.
//!   GAP (partial): the copy's added Spirit type and its embedded
//!   "leaves the battlefield → return the exiled card" trigger are not
//!   attached to the CopyPermanent token.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hofri Ghostforge");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    // "another nontoken creature you control"
    let dying_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: dying_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: copy_and_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn copy_and_exile(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(dying) = trig.dying_object() else { return Vec::new(); };
    // "another" — never trigger off Hofri's own death.
    if dying == trig.source {
        return Vec::new();
    }
    vec![
        Effect::CopyPermanent { target: dying },
        Effect::ExileFromGraveyard { target: dying },
    ]
}
