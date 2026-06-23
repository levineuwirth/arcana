//! Ogre Chitterlord — `{4}{R}{R}` 6/5 Creature — Ogre Warrior.
//! * Menace (keyword line).
//! * "Whenever this creature enters or attacks, create two 1/1 black Rat
//!   creature tokens with 'This token can't block.' Then if you control five
//!   or more Rats, each Rat you control gets +2/+0 until end of turn."
//!   "enters or attacks" decomposes into two triggers sharing one resolver.
//!   GAP (partial): the Rat tokens' printed "This token can't block." inner
//!   ability is not attached (TokenDefinition abilities surface for that is
//!   not available here) — the bare 1/1 black Rats are minted faithfully.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ogre Chitterlord");
    let ogre = reg.interner_mut().intern("Ogre");
    let warrior = reg.interner_mut().intern("Warrior");
    // Pre-intern the Rat subtype so the resolver can rebuild it via lookup.
    let _rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_rats_then_buff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_rats_then_buff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_rats_then_buff(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let rat = reg.interner().lookup("Rat").unwrap_or_default();
    let mut rat_subtypes = SubtypeSet::default();
    rat_subtypes.0.insert(rat);
    let rat_token = TokenDefinition {
        name: rat,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: rat_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };

    let mut effects = vec![
        Effect::CreateToken { controller: trig.controller, token: rat_token.clone() },
        Effect::CreateToken { controller: trig.controller, token: rat_token },
    ];

    // "Then if you control five or more Rats" — count current Rats you control
    // plus the two just being created.
    let rat_filter = script::subtype_filter(reg, "Rat")
        .controlled_by(ControllerConstraint::You);
    let current = script::count_matching(state, &rat_filter, trig.controller);
    if current + 2 >= 5 {
        let ids = script::ids_matching(state, &rat_filter, trig.controller);
        effects.push(Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: 2,
                toughness: 0,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        });
    }

    effects
}
