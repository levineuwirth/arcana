//! Flash Thompson, Spider-Fan — `{1}{W}` 2/2 Legendary Human Citizen
//! with Flash.
//! "When Flash Thompson enters, choose one or both —
//!  • Heckle — Tap target creature.
//!  • Hero Worship — Untap target creature."
//!
//! GAP (fidelity): triggered abilities have no modal dispatch (modal
//! is a SpellAbilityDef-only feature), so the "choose one or both"
//! selection is not expressible. Wired as both modes firing: tap one
//! target creature and untap a second target creature. Both individual
//! effects (Tap / Untap) are faithful; the player simply can't decline
//! a mode or repeat-target a single mode.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flash Thompson, Spider-Fan");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: tap_then_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement::target_creature(),
                    TargetRequirement::target_creature(),
                ],
            }),
    )
}

fn tap_then_untap(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = Vec::new();
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        out.push(Effect::Tap { target: *id });
    }
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.get(1) {
        out.push(Effect::Untap { target: *id });
    }
    out
}
