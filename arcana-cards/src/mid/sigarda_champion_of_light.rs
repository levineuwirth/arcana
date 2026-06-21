//! Sigarda, Champion of Light — `{1}{G}{W}{W}` 4/4 Legendary Angel.
//! Flying, trample.
//! "Humans you control get +1/+1." (static anthem — GAP'd.)
//! Coven — Whenever Sigarda attacks, if you control three or more
//! creatures with different powers, look at the top five cards of your
//! library. You may reveal a Human creature card from among them and put
//! it into your hand. Put the rest on the bottom in a random order.
//!
//! Flying and Trample are base keywords. The Humans anthem is a static
//! continuous ability with no expressible primitive, GAP'd. The Coven
//! trigger's intervening-if ("three or more creatures with different
//! powers") cannot be expressed with the available conditions, so it is
//! GAP'd (fires unconditionally); the dig itself is DigTopN over the top
//! five, taking a Human creature card to hand and bottoming the rest.

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sigarda, Champion of Light");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP (static): "Humans you control get +1/+1." No static-anthem
    // primitive in the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            // GAP (intervening-if): "if you control three or more creatures
            // with different powers" — no different-powers condition helper;
            // fires unconditionally.
            intervening_if: None,
            effect: coven_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn coven_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut filter = ObjectFilter::permanent().with_types(TypeLine::CREATURE.into());
    if let Some(sym) = reg.interner().lookup("Human") {
        filter = filter.with_subtype_sym(sym);
    }
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 5,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
