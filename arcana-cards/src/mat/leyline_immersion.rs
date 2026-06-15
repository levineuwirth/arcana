//! Leyline Immersion — `{3}{G}` enchantment — Aura.
//! "Enchant legendary creature. Enchanted creature has ward {2} and
//!  \"{T}: Add five mana in any combination of colors. Spend this mana only
//!  to cast spells.\""
//!
//! Keyword + host-mana-ability Aura. Ward {2} grants via `attached_keyword`.
//! The granted host mana ability ('add five mana in any combination of
//! colors, restricted to casting spells') has no expressible mana-production
//! effect (arbitrary chosen-color amount + spend restriction) and is GAPped.

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
    let name = reg.interner_mut().intern("Leyline Immersion");
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
            // NOTE: 'legendary creature' wording approximated by caster's choice
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_ward,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_ward(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted host mana ability ('{T}: add five mana in any combination
    // of colors, spend only to cast spells') — not expressible.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
