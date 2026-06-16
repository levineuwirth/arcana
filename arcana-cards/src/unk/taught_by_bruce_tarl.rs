//! Taught by Bruce Tarl — `{3}{R}` enchantment — Aura.
//! "Commander enchantment. When you cast Taught by Bruce Tarl, draw a card.
//!  Enchanted creature has double strike and protection from Oxen."
//!
//! Buff Aura. Double strike is an `attached_keyword(DoubleStrike)`; protection
//! from Oxen is `attached_keyword(Protection(CreatureType(Ox)))` — both ETB
//! installs that follow the host. The "Commander enchantment" command-zone
//! targeting rules and the "when you cast, draw a card" cast trigger are outside
//! the demonstrated aura surface — GAP.

use arcana_core::effects::{Effect, KeywordAbility, ProtectionQuality};
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
    let name = reg.interner_mut().intern("Taught by Bruce Tarl");
    let aura = reg.interner_mut().intern("Aura");
    // Intern the Ox creature type so the effect fn can look it back up.
    let _ox = reg.interner_mut().intern("Ox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    // GAP: "Commander enchantment" command-zone targeting; "When you cast …, draw a card."
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ox = reg.interner().lookup("Ox").expect("Ox interned in register");
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::DoubleStrike,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Protection(ProtectionQuality::CreatureType(ox)),
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
