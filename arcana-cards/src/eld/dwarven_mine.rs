//! Dwarven Mine — land — Mountain.
//! "({T}: Add {R}.)" / "This land enters tapped unless you control three or
//! more other Mountains." / "When this land enters untapped, create a 1/1
//! red Dwarf creature token." The conditional enters-tapped clause is a GAP
//! (the land enters untapped here), and the ETB trigger's "enters untapped"
//! condition is likewise noted — it fires on every entry.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Mine");
    let mountain = reg.interner_mut().intern("Mountain");
    let _dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mountain);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    // GAP: "enters tapped unless you control three or more other Mountains"
    // — conditional enters-tapped is not expressible (EntersWithSpec::Tapped
    // is unconditional); the land enters untapped here.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_mana,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dwarf_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_red_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn etb_dwarf_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "When this land enters UNTAPPED" — the entered-untapped condition
    // is not checkable here; the trigger fires on every entry (consistent
    // with the GAP'd conditional enters-tapped clause, under which this land
    // always enters untapped).
    let dwarf = reg.interner().lookup("Dwarf").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(d) = reg.interner().lookup("Dwarf") {
        subtypes.0.insert(d);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dwarf,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
