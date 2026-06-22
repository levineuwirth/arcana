//! Thalia and The Gitrog Monster — `{1}{W}{B}{G}` 4/4 Legendary Creature —
//! Human Frog Horror.
//! First strike, deathtouch.
//! You may play an additional land on each of your turns.
//! Creatures and nonbasic lands your opponents control enter tapped.
//! Whenever Thalia and The Gitrog Monster attacks, sacrifice a creature or
//! land, then draw a card.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Thalia and The Gitrog Monster");
    let human = reg.interner_mut().intern("Human");
    let frog = reg.interner_mut().intern("Frog");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(frog);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: "You may play an additional land on each of your turns." is a static
    // permission with no trigger word/activation cost; not expressible.
    // GAP: "Creatures and nonbasic lands your opponents control enter tapped."
    // is a static enters-tapped replacement; not expressible in this shape.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_sac_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_sac_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "sacrifice a creature or land, then draw a card"
    let creature_or_land = ObjectFilter::permanent()
        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::LAND))
        .controlled_by(ControllerConstraint::You);
    vec![Effect::Sequence(vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: creature_or_land,
            count: 1,
        },
        Effect::DrawCards { player: trig.controller, count: 1 },
    ])]
}
