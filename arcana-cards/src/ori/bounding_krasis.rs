//! Bounding Krasis — `{1}{G}{U}` 3/3 Fish Lizard with Flash.
//!
//! Oracle:
//!  * Flash.
//!  * When this creature enters, you may tap or untap target creature.
//!
//! Flash is wired. The ETB targets a creature, but its payoff is a player
//! choice between tapping and untapping the target ("you may tap or untap")
//! with no single Effect that expresses the binary choice — the effect is GAP'd
//! while the trigger and target are recorded.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bounding Krasis");
    let fish = reg.interner_mut().intern("Fish");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_tap_or_untap,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn etb_tap_or_untap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may tap or untap target creature" — a player choice between
    // Effect::Tap and Effect::Untap on the same target; no single Effect
    // expresses the binary may-choice.
    Vec::new()
}
