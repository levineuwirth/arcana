//! The Disciple of Vess — `{2}{B}{B}` 2/2 black Legendary Human Cleric.
//!
//! Oracle:
//! * "Whenever The Disciple of Vess attacks, you may put a planeswalker
//!   card from your hand onto the battlefield. If you don't, create a
//!   token that's a copy of a Liliana planeswalker chosen at random."
//!
//! The attack trigger lets you put a planeswalker from hand onto the
//! battlefield. The "if you don't, create a copy of a Liliana chosen at
//! random" branch is GAP'd: token-copy by card NAME chosen at random
//! requires a registry-by-name lookup that `Effect::CopyPermanent`
//! (which copies an existing permanent, not a named card) cannot
//! express.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("The Disciple of Vess");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may put a planeswalker card from your hand onto the
    // battlefield." (PutFromHandOntoBattlefield posts an optional pick.)
    //
    // GAP: "If you don't, create a token that's a copy of a Liliana
    // planeswalker chosen at random" — token-copy of a card chosen by
    // name at random has no expressible primitive.
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::PLANESWALKER.into()),
        tapped: false,
    }]
}
