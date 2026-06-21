//! Danitha, Benalia's Hope — `{4}{W}` 4/4 Legendary Human Knight with
//! First strike, Vigilance, and Lifelink. "When Danitha enters, you may
//! put an Aura or Equipment card from your hand or graveyard onto the
//! battlefield attached to Danitha."

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Danitha, Benalia's Hope");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let _aura = reg.interner_mut().intern("Aura");
    let _equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_put_aura_or_equipment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_put_aura_or_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: PutFromHandOntoBattlefield posts a may-pick over Aura/Equipment
    // cards in hand only.
    // GAP: the "or graveyard" source is not covered (no graveyard variant of
    // this primitive), and the "attached to Danitha" auto-attach on arrival
    // is not part of the put primitive.
    let mut any: Vec<arcana_core::types::SmallString> = Vec::new();
    if let Some(aura) = reg.interner().lookup("Aura") {
        any.push(aura);
    }
    if let Some(equipment) = reg.interner().lookup("Equipment") {
        any.push(equipment);
    }
    let filter = ObjectFilter::default().with_subtypes_any(any);
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter,
        tapped: false,
    }]
}
