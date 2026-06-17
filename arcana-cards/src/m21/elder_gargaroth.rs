//! Elder Gargaroth — `{3}{G}{G}` 6/6 Beast with Vigilance, Reach,
//! Trample. "Whenever this creature attacks or blocks, choose one —
//! create a 3/3 green Beast token; you gain 3 life; draw a card."
//!
//! The keyword line is fully modeled. The attacks-or-blocks modal
//! trigger is GAP'd: there is no attacks-or-blocks TriggerCondition and
//! "choose one" modality is only expressible on spell abilities, not on
//! triggered abilities, in the supported surface.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Elder Gargaroth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Reach,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Closest condition; the "or blocks" half and the modal
                // choice are GAP'd in the effect body.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_or_blocks_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_or_blocks_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one" modal on a triggered ability (and the "or blocks"
    // half) is not expressible in the supported surface.
    Vec::new()
}
