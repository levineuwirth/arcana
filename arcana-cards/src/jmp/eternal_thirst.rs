//! Eternal Thirst — `{1}{B}` enchantment — Aura.
//! "Enchant creature. Enchanted creature has lifelink and 'Whenever a
//!  creature an opponent controls dies, put a +1/+1 counter on this
//!  creature.'"
//!
//! Buff Aura: grants lifelink via an ETB-installed `attached_keyword`.
//! The granted triggered ability ("whenever a creature an opponent
//! controls dies …") is keyed on a board-wide opponent-creature death,
//! not a `Self*` condition on the host, so it cannot be expressed with
//! the AttachedCreatureDoes wrapper — GAP'd.

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
    let name = reg.interner_mut().intern("Eternal Thirst");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted "whenever a creature an opponent controls dies, +1/+1
    // counter on this creature" is a board-wide death trigger, not a Self*
    // condition wrappable by AttachedCreatureDoes — only lifelink expressed.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Lifelink,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
