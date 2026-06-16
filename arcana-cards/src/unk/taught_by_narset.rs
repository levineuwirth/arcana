//! Taught by Narset — `{1}{U}` enchantment — Aura.
//! "Commander enchantment. When you cast Taught by Narset, draw a card.
//!  Enchanted creature has prowess and ward {2}."
//!
//! Partial: the Ward {2} grant is the canonical `attached_keyword`
//! install. GAP'd: Prowess (not in the usable keyword surface), the
//! cast trigger "when you cast ~, draw a card" (a cast trigger, not a
//! host Self* condition), and the Commander-enchantment cast-from-command-
//! zone wording (no engine support — approximated as enchant creature).

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
    let name = reg.interner_mut().intern("Taught by Narset");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: Commander-enchantment / cast-from-command-zone wording
        // approximated by enchant creature.
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
    // GAP: Prowess (not in usable keyword surface) and the cast-trigger
    // card draw ("when you cast ~, draw a card") are not expressible here.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
