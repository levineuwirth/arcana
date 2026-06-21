//! Tervigon — `{X}{1}{G}` 0/0 Tyranid.
//!
//! Rules text:
//! * Ravenous (This creature enters with X +1/+1 counters on it. If X is 5 or
//!   more, draw a card when it enters.)
//! * Trample
//! * Spawn Termagants — Whenever this creature deals combat damage to a player,
//!   create that many 1/1 green Tyranid creature tokens.
//!
//! Trample is faithful. The combat-damage trigger creates that-many tokens by
//! reading the damage amount at resolution. Ravenous is GAP'd: it is not a
//! KeywordAbility variant, and "enters with X +1/+1 counters / draw on X>=5"
//! relies on the X paid to cast, which has no demonstrated cast-time hook.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tervigon");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: Ravenous is not a KeywordAbility variant; "enters with X +1/+1
        //      counters; draw if X>=5" has no demonstrated cast-X hook.
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: spawn_termagants,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn spawn_termagants(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    let tyranid = reg.interner().lookup("Tyranid").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);
    let token = TokenDefinition {
        name: tyranid,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
