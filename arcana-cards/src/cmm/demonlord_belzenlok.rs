//! Demonlord Belzenlok — `{4}{B}{B}` 6/6 Legendary Elder Demon with
//! Flying and Trample.
//! "When Demonlord Belzenlok enters, exile cards from the top of your
//! library until you exile a nonland card, then put that card into your
//! hand. If the card's mana value is 4 or greater, repeat this process.
//! Demonlord Belzenlok deals 1 damage to you for each card put into
//! your hand this way."
//!
//! Flying and Trample are base keywords. The ETB is a partial: it
//! reveals until the first nonland card and puts that one card into
//! hand via `RevealUntil`. The conditional repeat ("if mana value is 4
//! or greater, repeat") and the self-damage-per-card-drawn count have
//! no expressible primitive — GAP'd (a single iteration, no self-
//! damage). The non-matching tail goes to the graveyard as a documented
//! approximation.

use arcana_core::effects::{DigRest, Effect, KeywordAbility, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Demonlord Belzenlok");
    let elder = reg.interner_mut().intern("Elder");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_nonland(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "if mana value 4+, repeat" loop and the "deals 1 damage to
    // you for each card put into hand this way" self-damage count are not
    // expressible — only the first nonland is taken to hand.
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::new().without_types(TypeLine::LAND.into()),
        found_dest: RevealDest::Hand,
        rest: DigRest::Graveyard,
        max_reveal: None,
    }]
}
