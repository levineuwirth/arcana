//! Alien Symbiosis — `{1}{B}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1, has menace, and is a
//!  Symbiote in addition to its other types. You may cast this card from your
//!  graveyard by discarding a card in addition to paying its other costs."
//!
//! ETB installs +1/+1 (`attached_pt`), menace (`attached_keyword`), and the
//! Symbiote subtype (`attached_subtypes`). The graveyard-cast-by-discard
//! alternate cost is GAP'd (not expressible).

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
    let name = reg.interner_mut().intern("Alien Symbiosis");
    let aura = reg.interner_mut().intern("Aura");
    let _symbiote = reg.interner_mut().intern("Symbiote");
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
    // GAP: "cast from your graveyard by discarding a card" alternate cost.
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut symbiote_set = SubtypeSet::default();
    if let Some(sym) = reg.interner().lookup("Symbiote") {
        symbiote_set.0.insert(sym);
    }
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt(
                trig.source,
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Menace,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_subtypes(
                trig.source,
                symbiote_set,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
