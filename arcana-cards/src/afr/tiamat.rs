//! Tiamat — `{2}{W}{U}{B}{R}{G}` 7/7 Legendary Dragon God with Flying.
//! When Tiamat enters, if you cast it, search your library for up to five Dragon
//! cards not named Tiamat that each have different names, reveal them, put them
//! into your hand, then shuffle.
//!
//! GAP (partial): the "up to five … each have different names" multi-search and
//! the "not named Tiamat" exclusion are not expressible — `TutorToHand` searches
//! for ONE matching card with a single `ObjectFilter`. Rendered as a single
//! "search for a Dragon card, reveal, to hand". The "if you cast it" cast-cause
//! intervening-if has no `conditions::` helper, so it fires unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tiamat");
    let dragon = reg.interner_mut().intern("Dragon");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(
            ManaCost::parse("{2}{W}{U}{B}{R}{G}").expect("valid cost"),
        ),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: intervening-if "if you cast it" has no conditions:: helper;
            // fires unconditionally.
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: search_dragons,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn search_dragons(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "up to five … different names" multi-search and "not named Tiamat"
    // exclusion not expressible; single-card Dragon tutor as best effort.
    let filter = script::subtype_filter(reg, "Dragon");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}
