//! Zurgo, Thunder's Decree — `{R}{W}{B}` 2/4 legendary Orc Warrior.
//! "Mobilize 2 (Whenever this creature attacks, create two tapped and
//!  attacking 1/1 red Warrior creature tokens. Sacrifice them at the
//!  beginning of the next end step.)"
//! "During your end step, Warrior tokens you control have 'This token can't
//!  be sacrificed.'" (a continuous static — GAP'd)
//!
//! Mobilize is not an expressible KeywordAbility variant, so its reminder
//! text is decomposed into a SelfAttacks trigger that creates two tapped
//! and attacking Warrior tokens. The "sacrifice them at the next end step"
//! rider can't be attached (the minted token ids aren't visible to the
//! resolver, so DelayedAction can't target them) and is GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zurgo, Thunder's Decree");
    let orc = reg.interner_mut().intern("Orc");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "During your end step, Warrior tokens you control have 'This
    // token can't be sacrificed.'" — a continuous granted-static; not
    // expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: mobilize_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn mobilize_two(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let make_token = || {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(warrior);
        Effect::CreateTokenTappedAttacking {
            controller: trig.controller,
            token: TokenDefinition {
                name: warrior,
                colors: ColorSet::red(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    };
    // GAP: "Sacrifice them at the beginning of the next end step" — the
    // minted token ids aren't visible to the resolver, so the delayed
    // sacrifice can't be scheduled.
    vec![make_token(), make_token()]
}
