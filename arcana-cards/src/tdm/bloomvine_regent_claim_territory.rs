//! Bloomvine Regent // Claim Territory — `{3}{G}{G}` 4/5 green Dragon.
//! Flying. "Whenever this creature or another Dragon you control enters,
//! you gain 3 life."
//! Omen face "Claim Territory" (`{2}{G}` Sorcery):
//! "Search your library for up to two basic Forest cards, reveal them,
//! put one onto the battlefield tapped and the other into your hand,
//! then shuffle."
//! GAP: Omen type face — using Adventure shape (both are alternate-cast faces).
//! GAP: "put one to battlefield tapped" — TutorToBattlefield with tapped:true
//! handles one Forest. Two-Forest split not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloomvine Regent");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Claim Territory");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid adv cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Search your library for up to two basic Forest cards, reveal them, put one onto the battlefield tapped and the other into your hand, then shuffle.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: claim_territory,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: dragon_etb_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn dragon_etb_gain_life(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let entering = trig.entering_object().unwrap_or(trig.source);
    // Only trigger for Dragons (self or other Dragon)
    let dragon_filter = script::subtype_filter(reg, "Dragon");
    let is_dragon = script::ids_matching(state, &dragon_filter, trig.controller)
        .contains(&entering)
        || entering == trig.source;
    if is_dragon {
        vec![Effect::GainLife { player: trig.controller, amount: 3 }]
    } else {
        Vec::new()
    }
}

fn claim_territory(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Best-effort: put one basic Forest to battlefield tapped.
    // GAP: second Forest to hand not modeled; "up to two" split not expressible
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
        tapped: true,
    }]
}
