//! Shelob, Child of Ungoliant — `{4}{B}{G}` 8/8 Legendary Spider Demon with
//! Deathtouch and Ward {2}.
//!
//! "Other Spiders you control have deathtouch and ward {2}." — WIRED as two
//!  ETB-installed `ContinuousEffect::filtered_keyword` grants (Spiders you
//!  control gain Deathtouch, and Spiders you control gain Ward {2}), both
//!  lasting while Shelob is on the battlefield (glorious_anthem precedent). The
//!  filter matches all your Spiders; the "OTHER" self-inclusion is the
//!  documented minor fidelity gap (Shelob already has both keywords anyway).
//! "Whenever another creature dealt damage this turn by a Spider you controlled
//!  dies, create a token that's a copy of that creature, except it's a Food
//!  artifact ... and it loses all other card types." — the damaged-by-a-Spider
//!  this-turn history filter and the copy-with-Food-modifications token payload
//!  are not expressible (GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shelob, Child of Ungoliant");
    let spider = reg.interner_mut().intern("Spider");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![
            KeywordAbility::Deathtouch,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    // GAP: "Whenever another creature dealt damage this turn by a Spider you
    //   controlled dies, create a Food token copy of it (losing all other card
    //   types)" — the damaged-by-a-Spider this-turn history filter and the
    //   copy-with-Food-modifications payload are not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            // "Other Spiders you control have deathtouch and ward {2}." installed
            // on ETB as two filtered keyword grants.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_spider_anthems,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "Spiders you control have deathtouch" and "Spiders you control have
/// ward {2}" anchored to Shelob, lasting while it remains on the battlefield.
fn install_spider_anthems(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spider = reg.interner().lookup("Spider").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(spider);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter.clone(),
                KeywordAbility::Deathtouch,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter,
                KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
