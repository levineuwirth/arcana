//! Autarch Mammoth — `{4}{G}{G}` 5/5 Elephant Mount.
//! "When this creature enters and whenever it attacks while saddled,
//! create a 3/3 green Elephant creature token."
//! Saddle 5.
//!
//! Saddle is NOT in the usable keyword surface (no Saddle keyword
//! variant / no "while saddled" state predicate), so it is GAP'd. The
//! ETB half of the combined trigger is wired faithfully (create a 3/3
//! green Elephant). The attack half is GAP'd because it is gated on the
//! unmodeled saddled state.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Autarch Mammoth");
    let elephant = reg.interner_mut().intern("Elephant");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Saddle 5 — Saddle is not in the usable keyword surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // ETB half of the combined trigger. (The "whenever it attacks
            // while saddled" half is GAP'd — no saddled-state predicate.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_elephant,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_elephant(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let elephant = reg.interner().lookup("Elephant").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: arcana_core::effects::TokenDefinition {
            name: elephant,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
