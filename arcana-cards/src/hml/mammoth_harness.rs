//! Mammoth Harness — `{3}{G}` enchantment — Aura.
//! "Enchant creature. Enchanted creature loses flying. Whenever enchanted
//!  creature blocks or becomes blocked by a creature, the other creature
//!  gains first strike until end of turn."
//!
//! Keyword-removal Aura. ETB installs `attached_loses_keyword(Flying)` on
//! the host. The combat-pairing host trigger ("the OTHER creature gains
//! first strike") needs the paired combatant, which isn't reachable from
//! the trigger accessors — omitted (GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mammoth Harness");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "whenever enchanted creature blocks or becomes blocked by a
    // creature, the OTHER creature gains first strike" — needs the paired
    // combatant, not reachable from trigger accessors.
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_loses_keyword(
            trig.source,
            KeywordAbility::Flying,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
