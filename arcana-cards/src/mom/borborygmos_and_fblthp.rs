//! Borborygmos and Fblthp — `{2}{G}{U}{R}` 6/5 Legendary Cyclops
//! Homunculus.
//!
//! Oracle:
//! * Whenever Borborygmos and Fblthp enters or attacks, draw a card,
//!   then you may discard any number of land cards. When you discard
//!   one or more cards this way, Borborygmos and Fblthp deals twice
//!   that much damage to target creature. — modeled as two triggered
//!   abilities (one for "enters", one for "attacks"; there is no
//!   combined enters-or-attacks condition). Each draws a card and posts
//!   the optional discard-any-number-of-lands. GAP: the reflexive "when
//!   you discard one or more this way, deal twice that much to target
//!   creature" sub-trigger needs the count discarded during this
//!   resolution and a reflexive target — not expressible.
//! * {1}{U}: Put Borborygmos and Fblthp into its owner's library third
//!   from the top. — GAP: no "third from the top" library-placement
//!   primitive (only top / bottom).

use arcana_core::effects::{Effect, PickAction};
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
    let name = reg.interner_mut().intern("Borborygmos and Fblthp");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyclops);
    subtypes.0.insert(homunculus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: draw_then_discard_lands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: draw_then_discard_lands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_then_discard_lands(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the reflexive "When you discard one or more cards this way,
    // deal twice that much damage to target creature" is not modeled —
    // no accessor for cards discarded during this resolution.
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::ChooseAnyNumberFromZone {
            chooser: trig.controller,
            zone: Zone::Hand(trig.controller),
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            action: PickAction::Discard,
        },
    ])]
}
