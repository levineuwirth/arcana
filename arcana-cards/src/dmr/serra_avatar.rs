//! Serra Avatar — `{4}{W}{W}{W}` */* Avatar.
//! "Serra Avatar's power and toughness are each equal to your life
//! total." (GAP — a characteristic-defining static, not a triggered/
//! activated ability.)
//! "When Serra Avatar is put into a graveyard from anywhere, shuffle it
//! into its owner's library." (effect GAP — no shuffle-self-into-library
//! effect; SelfDies is the closest available trigger.)
//!
//! P/T is `*/*` → PtValue::Star on both. The dies trigger is wired but
//! its body is GAP'd.

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
    let name = reg.interner_mut().intern("Serra Avatar");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // */* — power and toughness are characteristic-defined (= life
        // total). GAP: the CDA that sets them to life total is a static.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // Closest available trigger: SelfDies (battlefield→graveyard);
            // oracle says "from anywhere".
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: shuffle_into_library,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn shuffle_into_library(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "shuffle it into its owner's library" — no effect to move this
    // card from the graveyard into the library and shuffle.
    Vec::new()
}
