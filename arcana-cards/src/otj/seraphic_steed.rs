//! Seraphic Steed — `{G}{W}` 2/2 Creature — Unicorn Mount.
//! First strike, lifelink. "Whenever this creature attacks while saddled,
//! create a 3/3 white Angel creature token with flying." Saddle 4.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seraphic Steed");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let mount = reg.interner_mut().intern("Mount");
    // Pre-intern the token subtype so the resolver's lookup succeeds.
    let _angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);
    subtypes.0.insert(mount);

    // GAP: keyword "Saddle 4" — not in the modeled KeywordAbility surface
    // (the saddle activated cost / "saddled" status are not expressible).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "while saddled" gating condition not expressible; trigger
            // fires on each attack and creates the Angel unconditionally.
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: make_angel,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_angel(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let angel = reg.interner().lookup("Angel").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: angel,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
