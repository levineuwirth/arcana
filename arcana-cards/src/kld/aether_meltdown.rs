//! Aether Meltdown — `{1}{U}` enchantment — Aura.
//! "Flash. Enchant creature or Vehicle. When this Aura enters, you get
//!  {E}{E} (two energy counters). Enchanted creature gets -4/-0."
//!
//! Flash buff (debuff) Aura. The Aura itself has Flash. ETB grants the
//! controller two energy counters (`Effect::GainEnergy`) and installs
//! `attached_pt(-4, 0)`. "Enchant creature or Vehicle" widened to any
//! creature (no Vehicle disjunction filter; Vehicles are artifacts that
//! become creatures).

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
    let name = reg.interner_mut().intern("Aether Meltdown");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        // NOTE: "enchant creature or Vehicle" widened to any creature.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_meltdown,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_meltdown(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(you) = state.objects.get(trig.source).map(|o| o.controller) {
        effects.push(Effect::GainEnergy { player: you, amount: 2 });
    }
    effects.push(Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            -4,
            0,
            Duration::WhileSourceOnBattlefield,
        ),
    });
    effects
}
