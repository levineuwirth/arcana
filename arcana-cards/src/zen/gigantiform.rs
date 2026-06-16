//! Gigantiform — `{3}{G}{G}` enchantment — Aura.
//! "Kicker {4}. Enchant creature. Enchanted creature has base power and
//!  toughness 8/8 and has trample. When this Aura enters, if it was kicked, you
//!  may search your library for a card named Gigantiform, put it onto the
//!  battlefield, then shuffle."
//!
//! ETB sets base P/T to 8/8 (`attached_set_pt`) and grants trample
//! (`attached_keyword`). Kicker and the if-kicked tutor-to-battlefield are
//! GAP'd (Kicker is not in the keyword surface; kicked-state ETB tutor is not
//! expressible).

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
    let name = reg.interner_mut().intern("Gigantiform");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        // GAP: Kicker {4} is not in the usable keyword surface.
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
    // GAP: "if it was kicked, search for a card named Gigantiform and put it
    // onto the battlefield" — kicked-state ETB tutor is not expressible.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_set_pt(
                trig.source,
                8,
                8,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_keyword(
                trig.source,
                KeywordAbility::Trample,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
