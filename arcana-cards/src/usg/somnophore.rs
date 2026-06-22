//! Somnophore — `{2}{U}{U}` 2/2 Illusion with Flying.
//! "Whenever this creature deals damage to a player, tap target creature
//! that player controls. That creature doesn't untap during its
//! controller's untap step for as long as this creature remains on the
//! battlefield."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Somnophore");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: tap_target_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // Fidelity GAP: "that player controls" — the target filter
                // can't reference the dynamically damaged player, so this
                // targets any creature.
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn tap_target_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "doesn't untap ... for as long as this creature remains on the
    // battlefield" — no don't-untap-while-source-present effect exists; only
    // the one-shot tap is applied.
    vec![Effect::Tap { target: *id }]
}
