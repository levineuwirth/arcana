//! Meandered Towershell — `{G}` enchantment — Aura.
//! "Enchant creature. Enchanted creature has islandwalk and \"Whenever
//!  this creature attacks, exile it and Meandered Towershell. Return it to
//!  the battlefield ... tapped and attacking ... on your next turn, then
//!  return Meandered Towershell ... attached to that creature.\""
//!
//! Islandwalk is granted via an `attached_keyword(Landwalk(Island))`. The
//! granted self-exiling delayed-return-attacking ability has no expressible
//! primitive in the demonstrated API — GAP.

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
    let name = reg.interner_mut().intern("Meandered Towershell");
    let aura = reg.interner_mut().intern("Aura");
    // Interned here so the keyword-grant effect fn can `lookup` it.
    let _island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
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
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted "whenever it attacks, exile it and this Aura, return them
    // next turn tapped and attacking" — no expressible primitive.
    let island = reg.interner().lookup("Island").expect("Island interned");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Landwalk(island),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
